using System;
using System.Xml.Linq;

namespace LooCast.System.Serialization
{
    public sealed class LongPrimitiveSerializer : PrimitiveSerializer<long>
    {
        #region Constructors
        public LongPrimitiveSerializer() : base()
        {
            
        }
        #endregion

        #region Methods
        public override XElement Serialize(string name, long serializablePrimitive)
        {
            XElement serializedElement = new XElement(name, new XAttribute("Value", serializablePrimitive));

            return serializedElement;
        }

        public override long Deserialize(XElement serializedPrimitive)
        {
            XAttribute valueAttribute = serializedPrimitive.Attribute("Value");
            if (valueAttribute == null)
            {
                throw new ArgumentException($"Element '{serializedPrimitive.Name}' does not have a 'Value' attribute!");
            }

            if (!long.TryParse(valueAttribute.Value, out long deserializedValue))
            {
                throw new ArgumentException($"Element '{serializedPrimitive.Name}' with value '{valueAttribute.Value}' could not be parsed as an long!");
            }

            return deserializedValue;
        }
        #endregion
    }
}
