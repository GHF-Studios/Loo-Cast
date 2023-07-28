using System;
using System.Xml.Linq;

namespace LooCast.System.Serialization
{
    public sealed class FloatPrimitiveSerializer : PrimitiveSerializer<float>
    {
        #region Constructors
        public FloatPrimitiveSerializer() : base()
        {
            
        }
        #endregion

        #region Methods
        public override XElement Serialize(string name, float serializablePrimitive)
        {
            XElement serializedElement = new XElement(name, new XAttribute("Value", serializablePrimitive));

            return serializedElement;
        }

        public override float Deserialize(XElement serializedPrimitive)
        {
            XAttribute valueAttribute = serializedPrimitive.Attribute("Value");
            if (valueAttribute == null)
            {
                throw new ArgumentException($"Element '{serializedPrimitive.Name}' does not have a 'Value' attribute!");
            }

            if (!float.TryParse(valueAttribute.Value, out float deserializedValue))
            {
                throw new ArgumentException($"Element '{serializedPrimitive.Name}' with value '{valueAttribute.Value}' could not be parsed as an float!");
            }

            return deserializedValue;
        }
        #endregion
    }
}
