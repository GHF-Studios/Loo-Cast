using System;
using System.Xml.Linq;

namespace LooCast.System.Serialization
{
    public sealed class BytePrimitiveSerializer : PrimitiveSerializer<byte>
    {
        #region Constructors
        public BytePrimitiveSerializer() : base()
        {
            
        }
        #endregion

        #region Methods
        public override XElement Serialize(string name, byte serializablePrimitive)
        {
            XElement serializedElement = new XElement(name, new XAttribute("Value", serializablePrimitive));

            return serializedElement;
        }

        public override byte Deserialize(XElement serializedPrimitive)
        {
            XAttribute valueAttribute = serializedPrimitive.Attribute("Value");
            if (valueAttribute == null)
            {
                throw new ArgumentException($"Element '{serializedPrimitive.Name}' does not have a 'Value' attribute!");
            }

            if (!byte.TryParse(valueAttribute.Value, out byte deserializedValue))
            {
                throw new ArgumentException($"Element '{serializedPrimitive.Name}' with value '{valueAttribute.Value}' could not be parsed as an byte!");
            }

            return deserializedValue;
        }
        #endregion
    }
}
