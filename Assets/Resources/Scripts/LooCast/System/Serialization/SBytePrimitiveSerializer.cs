using System;
using System.Xml.Linq;

namespace LooCast.System.Serialization
{
    public sealed class SBytePrimitiveSerializer : PrimitiveSerializer<sbyte>
    {
        #region Constructors
        public SBytePrimitiveSerializer() : base()
        {
            
        }
        #endregion

        #region Methods
        public override XElement Serialize(string name, sbyte serializablePrimitive)
        {
            XElement serializedElement = new XElement(name, new XAttribute("Value", serializablePrimitive));

            return serializedElement;
        }

        public override sbyte Deserialize(XElement serializedPrimitive)
        {
            XAttribute valueAttribute = serializedPrimitive.Attribute("Value");
            if (valueAttribute == null)
            {
                throw new ArgumentException($"Element '{serializedPrimitive.Name}' does not have a 'Value' attribute!");
            }

            if (!sbyte.TryParse(valueAttribute.Value, out sbyte deserializedValue))
            {
                throw new ArgumentException($"Element '{serializedPrimitive.Name}' with value '{valueAttribute.Value}' could not be parsed as an sbyte!");
            }

            return deserializedValue;
        }
        #endregion
    }
}
