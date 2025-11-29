using System.Windows;
using RomanForge.UI.ViewModels;

namespace RomanForge.UI
{
    public partial class MainWindow : Window
    {
        public MainWindow()
        {
            InitializeComponent();
            DataContext = new ConverterViewModel();
        }
    }
}
