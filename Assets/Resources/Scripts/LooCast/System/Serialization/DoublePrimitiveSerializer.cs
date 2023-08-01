using System;
using System.Xml.Linq;

namespace LooCast.System.Serialization
{
    public sealed class DoublePrimitiveSerializer : PrimitiveSerializer<double>
    {
        #region Constructors
        public DoublePrimitiveSerializer() : base()
        {
            
        }
        #endregion

        #region Methods
        public override XElement Serialize(string name, double serializablePrimitive)
        {
            XElement serializedElement = new XElement(name, new XAttribute("Value", serializablePrimitive));

            return serializedElement;
        }

        public override double Deserialize(XElement serializedPrimitive)
        {
            XAttribute valueAttribute = serializedPrimitive.Attribute("Value");
            if (valueAttribute == null)
            {
                throw new ArgumentException($"Element '{serializedPrimitive.Name}' does not have a 'Value' attribute!");
            }

            if (!double.TryParse(valueAttribute.Value, out double deserializedValue))
            {
                throw new ArgumentException($"Element '{serializedPrimitive.Name}' with value '{valueAttribute.Value}' could not be parsed as an double!");
            }

            return deserializedValue;
        }
        #endregion
    }
}
