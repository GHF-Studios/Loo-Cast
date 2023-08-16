using System;

namespace LooCast.Command
{
    public sealed class Variable
    {
        #region Properties
        public string Name { get; set; }
        public TypeInfo Type { get; set; }
        public object Value { get; set; }
        #endregion

        #region Constructors
        public Variable(string name, TypeInfo type)
        {
            if (name == null)
            {
                throw new ArgumentNullException(nameof(name));
            }

            if (type == null)
            {
                throw new ArgumentNullException(nameof(type));
            }

            Name = name;
            Type = type;
            Value = null;
        }

        public Variable(string name, TypeInfo type, object value)
        {
            if (name == null)
            {
                throw new ArgumentNullException(nameof(name));
            }
            if (type == null)
            {
                throw new ArgumentNullException(nameof(type));
            }

            Name = name;
            Type = type;

            SetValue(value);
        }
        #endregion

        #region Methods
        public object GetValue()
        {
            return Value;
        }

        public void SetValue(object value)
        {
            if (value == null)
            {
                throw new ArgumentNullException(nameof(value));
            }

            if (!Type.IsAssignableFrom(value.GetType()))
            {
                throw new ArgumentException($"The provided value '{value}' is not of expected type '{Type.FullTypeName}'!");
            }

            Value = value;
        }
        #endregion
    }
}
