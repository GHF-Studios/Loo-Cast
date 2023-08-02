using System;
using System.Xml.Linq;

namespace LooCast.System.Serialization
{
    public sealed class LongSerializer : PrimitiveAttributeSerializer, IPrimitiveAttributeSerializer<long>
    {
        #region Static Properties
        public static LongSerializer Instance
        {
            get
            {
                if (instance == null)
                {
                    instance = new LongSerializer();
                }
                return instance;
            }
        }
        #endregion

        #region Static Fields
        private static LongSerializer instance;
        #endregion

        #region Constructors
        private LongSerializer() : base(typeof(long))
        {
        }
        #endregion

        #region Methods
        public void Serialize(string primitiveAttributeName, long primitiveAttribute, out XAttribute serializedPrimitiveAttribute)
        {
            serializedPrimitiveAttribute = new XAttribute(primitiveAttributeName, primitiveAttribute);
        }

        public void Deserialize(XAttribute serializedPrimitiveAttribute, out long primitiveAttribute)
        {
            if (serializedPrimitiveAttribute == null)
            {
                throw new ArgumentNullException(nameof(serializedPrimitiveAttribute));
            }

            if (!long.TryParse(serializedPrimitiveAttribute.Value, out primitiveAttribute))
            {
                throw new ArgumentException($"Attribute '{serializedPrimitiveAttribute.Name}' with value '{serializedPrimitiveAttribute.Value}' could not be parsed as a long!");
            }
        }
        #endregion

        #region Overrides
        public override void Serialize(string primitiveAttributeName, object primitiveAttribute, out XAttribute serializedPrimitiveAttribute) => Serialize(primitiveAttributeName, (long)primitiveAttribute, out serializedPrimitiveAttribute);
        public override void Deserialize(XAttribute serializedPrimitiveAttribute, out object primitiveAttribute) => Deserialize(serializedPrimitiveAttribute, out primitiveAttribute);
        #endregion
    }
}
