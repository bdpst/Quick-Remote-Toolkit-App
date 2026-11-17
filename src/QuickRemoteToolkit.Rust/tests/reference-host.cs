using System.IO;
using System.Windows;
using System.Windows.Controls;
using System.Windows.Media;
using System.Windows.Media.Imaging;
using System.Windows.Threading;
class Program {
 [STAThread]static void Main(){
 var app=new QuickRemoteToolkit.App.App();app.InitializeComponent();
 QuickRemoteToolkit.App.Services.ThemeService.Apply(Environment.GetEnvironmentVariable("QRT_QA_DARK")=="1");
 var scene=Environment.GetEnvironmentVariable("QRT_QA_SCENE")??"main";
 Window window=scene switch {
 "message" or "message-error"=>new QuickRemoteToolkit.App.SendMessageWindow("PC-DEMO"),
 "admin" or "admin-error"=>new QuickRemoteToolkit.App.LocalAdministratorsWindow("PC-DEMO"),
 _=>new QuickRemoteToolkit.App.MainWindow{Width=double.TryParse(Environment.GetEnvironmentVariable("QRT_QA_WIDTH"),out var width)?width:1500,Height=double.TryParse(Environment.GetEnvironmentVariable("QRT_QA_HEIGHT"),out var height)?height:920}
 };
 if(scene=="message-error")((TextBlock)window.FindName("StatusText")).Text="Укажите время показа от 1 до 3600 секунд.";
 if(scene=="admin-error")((TextBlock)window.FindName("StatusText")).Text="Введите учётную запись в формате ДОМЕН\\логин или ИМЯ-ПК\\логин.";
 if(scene=="context")window.Loaded+=(_,_)=>{var grid=(DataGrid)window.FindName("ClientsGrid");grid.SelectedIndex=0;grid.ContextMenu.PlacementTarget=grid;grid.ContextMenu.IsOpen=true;};
 window.Loaded+=(_,_)=>app.Dispatcher.BeginInvoke(DispatcherPriority.ApplicationIdle,new Action(()=>{
 try {
 var dpi=VisualTreeHelper.GetDpi(window);var visual=(FrameworkElement)VisualTreeHelper.GetChild(window,0);
 var image=new RenderTargetBitmap((int)Math.Round(visual.ActualWidth*dpi.DpiScaleX),(int)Math.Round(visual.ActualHeight*dpi.DpiScaleY),dpi.PixelsPerInchX,dpi.PixelsPerInchY,PixelFormats.Pbgra32);image.Render(visual);
 if(scene=="context") {var menu=((DataGrid)window.FindName("ClientsGrid")).ContextMenu;var menuImage=new RenderTargetBitmap((int)Math.Round(menu.ActualWidth*dpi.DpiScaleX),(int)Math.Round(menu.ActualHeight*dpi.DpiScaleY),dpi.PixelsPerInchX,dpi.PixelsPerInchY,PixelFormats.Pbgra32);menuImage.Render(menu);var menuEncoder=new PngBitmapEncoder();menuEncoder.Frames.Add(BitmapFrame.Create(menuImage));using var menuFile=File.Create(Environment.GetEnvironmentVariable("QRT_QA_IMAGE")+".menu.png");menuEncoder.Save(menuFile);}
 var encoder=new PngBitmapEncoder();encoder.Frames.Add(BitmapFrame.Create(image));using var file=File.Create(Environment.GetEnvironmentVariable("QRT_QA_IMAGE")!);encoder.Save(file);
 File.WriteAllText(Environment.GetEnvironmentVariable("QRT_QA_IMAGE")+".txt",$"Actual WPF window/client visual; reference 91689516c96d52877bba3480bf498b7d807ead64; fixture SettingsService path adapter and StartupUri removed for host only; scene={scene}; outer={window.ActualWidth}x{window.ActualHeight} DIP; client={visual.ActualWidth}x{visual.ActualHeight} DIP; dpi={dpi.PixelsPerInchX}; font={window.FontFamily}/{window.FontSize}; style={window.Style}");
 void Walk(DependencyObject root) {for(int i=0;i<VisualTreeHelper.GetChildrenCount(root);i++){var c=VisualTreeHelper.GetChild(root,i);if(c is TextBlock tb)File.AppendAllText(Environment.GetEnvironmentVariable("QRT_QA_IMAGE")+".txt",$"\nTextBlock {tb.Name} {tb.ActualWidth}x{tb.ActualHeight} {tb.FontFamily}/{tb.FontSize} {tb.Text}");if(c is DataGrid dg)File.AppendAllText(Environment.GetEnvironmentVariable("QRT_QA_IMAGE")+".txt",$"\nGRID {dg.Name} Clipboard={dg.ClipboardCopyMode} Selection={dg.SelectionMode} Reorder={dg.CanUserReorderColumns}");if(c is Control ctl)File.AppendAllText(Environment.GetEnvironmentVariable("QRT_QA_IMAGE")+".txt",$"\n{ctl.GetType().Name} {ctl.Name} {ctl.ActualWidth}x{ctl.ActualHeight} {ctl.FontFamily}/{ctl.FontSize} enabled={ctl.IsEnabled} style={ctl.Style}");Walk(c);}}Walk(window);
 }finally{window.Close();app.Shutdown();}
 }));
 app.Run(window);
 }
}
