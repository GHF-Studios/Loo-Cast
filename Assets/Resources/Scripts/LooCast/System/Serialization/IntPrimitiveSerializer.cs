using System;
using System.Xml.Linq;

namespace LooCast.System.Serialization
{
    public sealed class IntPrimitiveSerializer : PrimitiveSerializer<int>
    {
        #region Constructors
        public IntPrimitiveSerializer() : base()
        {
            
        }
        #endregion

        #region Methods
        public override XElement Serialize(string name, int serializablePrimitive)
        {
            XElement serializedElement = new XElement(name, new XAttribute("Value", serializablePrimitive));

            return serializedElement;
        }

        public override int Deserialize(XElement serializedPrimitive)
        {
            XAttribute valueAttribute = serializedPrimitive.Attribute("Value");
            if (valueAttribute == null)
            {
                throw new ArgumentException($"Element '{serializedPrimitive.Name}' does not have a 'Value' attribute!");
            }

            if (!int.TryParse(valueAttribute.Value, out int deserializedValue))
            {
                throw new ArgumentException($"Element '{serializedPrimitive.Name}' with value '{valueAttribute.Value}' could not be parsed as an int!");
            }

            return deserializedValue;
        }
        #endregion
    }
}
