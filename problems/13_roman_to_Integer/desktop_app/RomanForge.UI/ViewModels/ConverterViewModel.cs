using System;
using System.ComponentModel;
using System.Runtime.CompilerServices;
using System.Windows.Media;
using RomanForge.UI.Interop;
using RomanForge.UI.Validators;

namespace RomanForge.UI.ViewModels
{
    public class ConverterViewModel : INotifyPropertyChanged
    {
        private string _romanInput = string.Empty;
        private string _integerInput = string.Empty;
        private string _integerResult = "-";
        private string _romanResult = "-";
        private string _romanValidationText = string.Empty;
        private string _integerValidationText = string.Empty;
        private Brush _romanValidationColor = Brushes.Gray;
        private Brush _integerValidationColor = Brushes.Gray;

        public event PropertyChangedEventHandler? PropertyChanged;

        public string RomanInput
        {
            get => _romanInput;
            set
            {
                if (_romanInput != value)
                {
                    _romanInput = value;
                    OnPropertyChanged();
                    ConvertRomanToInteger();
                }
            }
        }

        public string IntegerInput
        {
            get => _integerInput;
            set
            {
                if (_integerInput != value)
                {
                    _integerInput = value;
                    OnPropertyChanged();
                    ConvertIntegerToRoman();
                }
            }
        }

        public string IntegerResult
        {
            get => _integerResult;
            set
            {
                _integerResult = value;
                OnPropertyChanged();
            }
        }

        public string RomanResult
        {
            get => _romanResult;
            set
            {
                _romanResult = value;
                OnPropertyChanged();
            }
        }

        public string RomanValidationText
        {
            get => _romanValidationText;
            set
            {
                _romanValidationText = value;
                OnPropertyChanged();
            }
        }

        public string IntegerValidationText
        {
            get => _integerValidationText;
            set
            {
                _integerValidationText = value;
                OnPropertyChanged();
            }
        }

        public Brush RomanValidationColor
        {
            get => _romanValidationColor;
            set
            {
                _romanValidationColor = value;
                OnPropertyChanged();
            }
        }

        public Brush IntegerValidationColor
        {
            get => _integerValidationColor;
            set
            {
                _integerValidationColor = value;
                OnPropertyChanged();
            }
        }

        private void ConvertRomanToInteger()
        {
            if (string.IsNullOrWhiteSpace(RomanInput))
            {
                IntegerResult = "-";
                RomanValidationText = string.Empty;
                RomanValidationColor = Brushes.Gray;
                return;
            }

            if (!RomanNumeralValidator.IsValidRoman(RomanInput))
            {
                IntegerResult = "Invalid";
                RomanValidationText = "✗ Invalid Roman numeral";
                RomanValidationColor = Brushes.Red;
                return;
            }

            try
            {
                var result = NativeEngine.RomanToInt(RomanInput);
                
                if (result.Success)
                {
                    IntegerResult = result.Value.ToString();
                    RomanValidationText = "✓ Valid";
                    RomanValidationColor = Brushes.Green;
                }
                else
                {
                    IntegerResult = "Error";
                    RomanValidationText = "✗ Conversion failed";
                    RomanValidationColor = Brushes.Red;
                }
            }
            catch (Exception ex)
            {
                IntegerResult = "Error";
                RomanValidationText = $"✗ {ex.Message}";
                RomanValidationColor = Brushes.Red;
            }
        }

        private void ConvertIntegerToRoman()
        {
            if (string.IsNullOrWhiteSpace(IntegerInput))
            {
                RomanResult = "-";
                IntegerValidationText = string.Empty;
                IntegerValidationColor = Brushes.Gray;
                return;
            }

            if (!int.TryParse(IntegerInput, out int value))
            {
                RomanResult = "Invalid";
                IntegerValidationText = "✗ Must be a number";
                IntegerValidationColor = Brushes.Red;
                return;
            }

            if (value < 1 || value > 3999)
            {
                RomanResult = "Out of Range";
                IntegerValidationText = "✗ Must be between 1-3999";
                IntegerValidationColor = Brushes.Red;
                return;
            }

            try
            {
                string? result = NativeEngine.IntToRoman(value);
                
                if (!string.IsNullOrEmpty(result))
                {
                    RomanResult = result;
                    IntegerValidationText = "✓ Valid";
                    IntegerValidationColor = Brushes.Green;
                }
                else
                {
                    RomanResult = "Error";
                    IntegerValidationText = "✗ Conversion failed";
                    IntegerValidationColor = Brushes.Red;
                }
            }
            catch (Exception ex)
            {
                RomanResult = "Error";
                IntegerValidationText = $"✗ {ex.Message}";
                IntegerValidationColor = Brushes.Red;
            }
        }

        protected void OnPropertyChanged([CallerMemberName] string? propertyName = null)
        {
            PropertyChanged?.Invoke(this, new PropertyChangedEventArgs(propertyName));
        }
    }
}
