using System;
using System.Xml.Linq;

namespace LooCast.System.Serialization
{
    public sealed class ShortPrimitiveSerializer : PrimitiveSerializer<short>
    {
        #region Constructors
        public ShortPrimitiveSerializer() : base()
        {
            
        }
        #endregion

        #region Methods
        public override XElement Serialize(string name, short serializablePrimitive)
        {
            XElement serializedElement = new XElement(name, new XAttribute("Value", serializablePrimitive));

            return serializedElement;
        }

        public override short Deserialize(XElement serializedPrimitive)
        {
            XAttribute valueAttribute = serializedPrimitive.Attribute("Value");
            if (valueAttribute == null)
            {
                throw new ArgumentException($"Element '{serializedPrimitive.Name}' does not have a 'Value' attribute!");
            }

            if (!short.TryParse(valueAttribute.Value, out short deserializedValue))
            {
                throw new ArgumentException($"Element '{serializedPrimitive.Name}' with value '{valueAttribute.Value}' could not be parsed as an short!");
            }

            return deserializedValue;
        }
        #endregion
    }
}
