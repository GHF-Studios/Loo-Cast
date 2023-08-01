using System;
using System.Xml.Linq;

namespace LooCast.System.Serialization
{
    public sealed class CharPrimitiveSerializer : PrimitiveSerializer<char>
    {
        #region Constructors
        public CharPrimitiveSerializer() : base()
        {
            
        }
        #endregion

        #region Methods
        public override XElement Serialize(string name, char serializablePrimitive)
        {
            XElement serializedElement = new XElement(name, new XAttribute("Value", serializablePrimitive));

            return serializedElement;
        }

        public override char Deserialize(XElement serializedPrimitive)
        {
            XAttribute valueAttribute = serializedPrimitive.Attribute("Value");
            if (valueAttribute == null)
            {
                throw new ArgumentException($"Element '{serializedPrimitive.Name}' does not have a 'Value' attribute!");
            }

            if (!char.TryParse(valueAttribute.Value, out char deserializedValue))
            {
                throw new ArgumentException($"Element '{serializedPrimitive.Name}' with value '{valueAttribute.Value}' could not be parsed as an char!");
            }

            return deserializedValue;
        }
        #endregion
    }
}
