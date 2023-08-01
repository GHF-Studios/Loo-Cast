using System;
using System.Xml.Linq;

namespace LooCast.System.Serialization
{
    public sealed class ULongPrimitiveSerializer : PrimitiveSerializer<ulong>
    {
        #region Constructors
        public ULongPrimitiveSerializer() : base()
        {
            
        }
        #endregion

        #region Methods
        public override XElement Serialize(string name, ulong serializablePrimitive)
        {
            XElement serializedElement = new XElement(name, new XAttribute("Value", serializablePrimitive));

            return serializedElement;
        }

        public override ulong Deserialize(XElement serializedPrimitive)
        {
            XAttribute valueAttribute = serializedPrimitive.Attribute("Value");
            if (valueAttribute == null)
            {
                throw new ArgumentException($"Element '{serializedPrimitive.Name}' does not have a 'Value' attribute!");
            }

            if (!ulong.TryParse(valueAttribute.Value, out ulong deserializedValue))
            {
                throw new ArgumentException($"Element '{serializedPrimitive.Name}' with value '{valueAttribute.Value}' could not be parsed as an ulong!");
            }

            return deserializedValue;
        }
        #endregion
    }
}
