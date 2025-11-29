using System.Windows;

namespace RomanForge.UI
{
    /// <summary>
    /// RomanForge Application Entry Point
    /// The Humanoid City's Gateway
    /// </summary>
    public partial class App : Application
    {
        protected override void OnStartup(StartupEventArgs e)
        {
            base.OnStartup(e);
            
            // Initialize application resources
            // Future: Load user preferences, themes, etc.
        }
    }
}
