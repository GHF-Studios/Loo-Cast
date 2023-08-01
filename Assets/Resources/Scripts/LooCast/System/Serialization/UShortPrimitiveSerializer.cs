using System;
using System.Xml.Linq;

namespace LooCast.System.Serialization
{
    public sealed class UShortPrimitiveSerializer : PrimitiveSerializer<ushort>
    {
        #region Constructors
        public UShortPrimitiveSerializer() : base()
        {
            
        }
        #endregion

        #region Methods
        public override XElement Serialize(string name, ushort serializablePrimitive)
        {
            XElement serializedElement = new XElement(name, new XAttribute("Value", serializablePrimitive));

            return serializedElement;
        }

        public override ushort Deserialize(XElement serializedPrimitive)
        {
            XAttribute valueAttribute = serializedPrimitive.Attribute("Value");
            if (valueAttribute == null)
            {
                throw new ArgumentException($"Element '{serializedPrimitive.Name}' does not have a 'Value' attribute!");
            }

            if (!ushort.TryParse(valueAttribute.Value, out ushort deserializedValue))
            {
                throw new ArgumentException($"Element '{serializedPrimitive.Name}' with value '{valueAttribute.Value}' could not be parsed as an ushort!");
            }

            return deserializedValue;
        }
        #endregion
    }
}
