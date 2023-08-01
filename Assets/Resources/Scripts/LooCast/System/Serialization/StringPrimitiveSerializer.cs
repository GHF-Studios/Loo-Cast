using System;
using System.Xml.Linq;

namespace LooCast.System.Serialization
{
    public sealed class StringPrimitiveSerializer : PrimitiveSerializer<string>
    {
        #region Constructors
        public StringPrimitiveSerializer() : base()
        {
            
        }
        #endregion

        #region Methods
        public override XElement Serialize(string name, string serializablePrimitive)
        {
            XElement serializedElement = new XElement(name, new XAttribute("Value", serializablePrimitive));

            return serializedElement;
        }

        public override string Deserialize(XElement serializedPrimitive)
        {
            XAttribute valueAttribute = serializedPrimitive.Attribute("Value");
            if (valueAttribute == null)
            {
                throw new ArgumentException($"Element '{serializedPrimitive.Name}' does not have a 'Value' attribute!");
            }

            return valueAttribute.Value;
        }
        #endregion
    }
}
