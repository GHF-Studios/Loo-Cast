using System;
using System.Xml.Linq;

namespace LooCast.System.Serialization
{
    public sealed class SByteSerializer : IPrimitiveAttributeSerializer<sbyte>
    {
        #region Static Properties
        public static SByteSerializer Instance
        {
            get
            {
                if (instance == null)
                {
                    instance = new SByteSerializer();
                }
                return instance;
            }
        }
        #endregion

        #region Static Fields
        private static SByteSerializer instance;
        #endregion

        #region Constructors
        private SByteSerializer() : base()
        {
        }
        #endregion

        #region Methods
        public void Serialize(string primitiveAttributeName, object primitiveAttribute, out XAttribute serializedPrimitiveAttribute) => Serialize(primitiveAttributeName, (sbyte)primitiveAttribute, out serializedPrimitiveAttribute);

        public void Serialize(string primitiveAttributeName, sbyte primitiveAttribute, out XAttribute serializedPrimitiveAttribute)
        {
            serializedPrimitiveAttribute = new XAttribute(primitiveAttributeName, primitiveAttribute);
        }

        public void Deserialize(XAttribute serializedPrimitiveAttribute, out object primitiveAttribute) => Deserialize(serializedPrimitiveAttribute, out primitiveAttribute);

        public void Deserialize(XAttribute serializedPrimitiveAttribute, out sbyte primitiveAttribute)
        {
            if (serializedPrimitiveAttribute == null)
            {
                throw new ArgumentNullException(nameof(serializedPrimitiveAttribute));
            }

            if (!sbyte.TryParse(serializedPrimitiveAttribute.Value, out primitiveAttribute))
            {
                throw new ArgumentException($"Attribute '{serializedPrimitiveAttribute.Name}' with value '{serializedPrimitiveAttribute.Value}' could not be parsed as an sbyte!");
            }
        }
        #endregion
    }
}
