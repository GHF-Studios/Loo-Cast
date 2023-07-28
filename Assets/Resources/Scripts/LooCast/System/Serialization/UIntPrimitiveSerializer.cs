using System;
using System.Xml.Linq;

namespace LooCast.System.Serialization
{
    public sealed class UIntPrimitiveSerializer : PrimitiveSerializer<uint>
    {
        #region Constructors
        public UIntPrimitiveSerializer() : base()
        {
            
        }
        #endregion

        #region Methods
        public override XElement Serialize(string name, uint serializablePrimitive)
        {
            XElement serializedElement = new XElement(name, new XAttribute("Value", serializablePrimitive));

            return serializedElement;
        }

        public override uint Deserialize(XElement serializedPrimitive)
        {
            XAttribute valueAttribute = serializedPrimitive.Attribute("Value");
            if (valueAttribute == null)
            {
                throw new ArgumentException($"Element '{serializedPrimitive.Name}' does not have a 'Value' attribute!");
            }

            if (!uint.TryParse(valueAttribute.Value, out uint deserializedValue))
            {
                throw new ArgumentException($"Element '{serializedPrimitive.Name}' with value '{valueAttribute.Value}' could not be parsed as an uint!");
            }

            return deserializedValue;
        }
        #endregion
    }
}
