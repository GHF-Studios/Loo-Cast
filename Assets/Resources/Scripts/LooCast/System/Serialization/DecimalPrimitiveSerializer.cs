using System;
using System.Xml.Linq;

namespace LooCast.System.Serialization
{
    public sealed class DecimalPrimitiveSerializer : PrimitiveSerializer<decimal>
    {
        #region Constructors
        public DecimalPrimitiveSerializer() : base()
        {
            
        }
        #endregion

        #region Methods
        public override XElement Serialize(string name, decimal serializablePrimitive)
        {
            XElement serializedElement = new XElement(name, new XAttribute("Value", serializablePrimitive));

            return serializedElement;
        }

        public override decimal Deserialize(XElement serializedPrimitive)
        {
            XAttribute valueAttribute = serializedPrimitive.Attribute("Value");
            if (valueAttribute == null)
            {
                throw new ArgumentException($"Element '{serializedPrimitive.Name}' does not have a 'Value' attribute!");
            }

            if (!decimal.TryParse(valueAttribute.Value, out decimal deserializedValue))
            {
                throw new ArgumentException($"Element '{serializedPrimitive.Name}' with value '{valueAttribute.Value}' could not be parsed as an decimal!");
            }

            return deserializedValue;
        }
        #endregion
    }
}
