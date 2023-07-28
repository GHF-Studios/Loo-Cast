using System;
using System.Xml.Linq;

namespace LooCast.System.Serialization
{
    public sealed class BoolPrimitiveSerializer : PrimitiveSerializer<bool>
    {
        #region Constructors
        public BoolPrimitiveSerializer() : base()
        {
            
        }
        #endregion

        #region Methods
        public override XElement Serialize(string name, bool serializablePrimitive)
        {
            XElement serializedElement = new XElement(name, new XAttribute("Value", serializablePrimitive));

            return serializedElement;
        }

        public override bool Deserialize(XElement serializedPrimitive)
        {
            XAttribute valueAttribute = serializedPrimitive.Attribute("Value");
            if (valueAttribute == null)
            {
                throw new ArgumentException($"Element '{serializedPrimitive.Name}' does not have a 'Value' attribute!");
            }

            if (!bool.TryParse(valueAttribute.Value, out bool deserializedValue))
            {
                throw new ArgumentException($"Element '{serializedPrimitive.Name}' with value '{valueAttribute.Value}' could not be parsed as an bool!");
            }

            return deserializedValue;
        }
        #endregion
    }
}
