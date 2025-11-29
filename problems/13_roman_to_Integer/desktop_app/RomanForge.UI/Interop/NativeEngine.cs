using System;
using System.Runtime.InteropServices;

namespace RomanForge.UI.Interop
{
    /// <summary>
    /// P/Invoke declarations for the Rust engine
    /// The bridge to the Titan-Machine Citadel
    /// </summary>
    public static class NativeEngine
    {
        private const string DllName = "roman_engine.dll";

        [StructLayout(LayoutKind.Sequential)]
        public struct ConversionResult
        {
            [MarshalAs(UnmanagedType.I1)]
            public bool Success;
            public int Value;
            public int ErrorCode;
        }

        /// <summary>
        /// Convert Roman numeral to integer
        /// Calls the Rust core engine
        /// </summary>
        [DllImport(DllName, CallingConvention = CallingConvention.Cdecl, CharSet = CharSet.Ansi)]
        private static extern ConversionResult roman_to_int(
            [MarshalAs(UnmanagedType.LPStr)] string romanStr);

        /// <summary>
        /// Convert integer to Roman numeral
        /// Calls the Rust core engine
        /// </summary>
        [DllImport(DllName, CallingConvention = CallingConvention.Cdecl)]
        private static extern IntPtr int_to_roman(int value);

        /// <summary>
        /// Free string allocated by Rust
        /// </summary>
        [DllImport(DllName, CallingConvention = CallingConvention.Cdecl)]
        private static extern void free_roman_string(IntPtr s);

        /// <summary>
        /// High-level wrapper for Roman to Integer conversion
        /// </summary>
        public static ConversionResult RomanToInt(string roman)
        {
            if (string.IsNullOrEmpty(roman))
            {
                return new ConversionResult
                {
                    Success = false,
                    Value = 0,
                    ErrorCode = 1
                };
            }

            return roman_to_int(roman.ToUpper());
        }

        /// <summary>
        /// High-level wrapper for Integer to Roman conversion
        /// </summary>
        public static string? IntToRoman(int value)
        {
            IntPtr ptr = int_to_roman(value);
            
            if (ptr == IntPtr.Zero)
                return null;

            try
            {
                return Marshal.PtrToStringAnsi(ptr);
            }
            finally
            {
                // Always free the Rust-allocated string
                free_roman_string(ptr);
            }
        }
    }
}
