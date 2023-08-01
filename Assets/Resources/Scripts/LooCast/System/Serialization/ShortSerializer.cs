using System;
using System.Xml.Linq;

namespace LooCast.System.Serialization
{
    public sealed class ShortSerializer : IPrimitiveAttributeSerializer<short>
    {
        #region Static Properties
        public static ShortSerializer Instance
        {
            get
            {
                if (instance == null)
                {
                    instance = new ShortSerializer();
                }
                return instance;
            }
        }
        #endregion

        #region Static Fields
        private static ShortSerializer instance;
        #endregion

        #region Constructors
        private ShortSerializer() : base()
        {
        }
        #endregion

        #region Methods
        public void Serialize(string primitiveAttributeName, object primitiveAttribute, out XAttribute serializedPrimitiveAttribute) => Serialize(primitiveAttributeName, (short)primitiveAttribute, out serializedPrimitiveAttribute);

        public void Serialize(string primitiveAttributeName, short primitiveAttribute, out XAttribute serializedPrimitiveAttribute)
        {
            serializedPrimitiveAttribute = new XAttribute(primitiveAttributeName, primitiveAttribute);
        }

        public void Deserialize(XAttribute serializedPrimitiveAttribute, out object primitiveAttribute) => Deserialize(serializedPrimitiveAttribute, out primitiveAttribute);

        public void Deserialize(XAttribute serializedPrimitiveAttribute, out short primitiveAttribute)
        {
            if (serializedPrimitiveAttribute == null)
            {
                throw new ArgumentNullException(nameof(serializedPrimitiveAttribute));
            }

            if (!short.TryParse(serializedPrimitiveAttribute.Value, out primitiveAttribute))
            {
                throw new ArgumentException($"Attribute '{serializedPrimitiveAttribute.Name}' with value '{serializedPrimitiveAttribute.Value}' could not be parsed as a short!");
            }
        }
        #endregion
    }
}
