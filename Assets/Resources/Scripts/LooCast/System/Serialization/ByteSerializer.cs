using System;
using System.Xml.Linq;

namespace LooCast.System.Serialization
{
    public sealed class ByteSerializer : IPrimitiveAttributeSerializer<byte>
    {
        #region Static Properties
        public static ByteSerializer Instance
        {
            get
            {
                if (instance == null)
                {
                    instance = new ByteSerializer();
                }
                return instance;
            }
        }
        #endregion

        #region Static Fields
        private static ByteSerializer instance;
        #endregion

        #region Constructors
        private ByteSerializer() : base()
        {
        }
        #endregion

        #region Methods
        public void Serialize(string primitiveAttributeName, object primitiveAttribute, out XAttribute serializedPrimitiveAttribute) => Serialize(primitiveAttributeName, (byte)primitiveAttribute, out serializedPrimitiveAttribute);

        public void Serialize(string primitiveAttributeName, byte primitiveAttribute, out XAttribute serializedPrimitiveAttribute)
        {
            serializedPrimitiveAttribute = new XAttribute(primitiveAttributeName, primitiveAttribute);
        }

        public void Deserialize(XAttribute serializedPrimitiveAttribute, out object primitiveAttribute) => Deserialize(serializedPrimitiveAttribute, out primitiveAttribute);

        public void Deserialize(XAttribute serializedPrimitiveAttribute, out byte primitiveAttribute)
        {
            if (serializedPrimitiveAttribute == null)
            {
                throw new ArgumentNullException(nameof(serializedPrimitiveAttribute));
            }

            if (!byte.TryParse(serializedPrimitiveAttribute.Value, out primitiveAttribute))
            {
                throw new ArgumentException($"Attribute '{serializedPrimitiveAttribute.Name}' with value '{serializedPrimitiveAttribute.Value}' could not be parsed as a byte!");
            }
        }
        #endregion
    }
}
