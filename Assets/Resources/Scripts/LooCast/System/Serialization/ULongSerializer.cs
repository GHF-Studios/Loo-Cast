using System;
using System.Xml.Linq;

namespace LooCast.System.Serialization
{
    public sealed class ULongSerializer : PrimitiveAttributeSerializer, IPrimitiveAttributeSerializer<ulong>
    {
        #region Static Properties
        public static ULongSerializer Instance
        {
            get
            {
                if (instance == null)
                {
                    instance = new ULongSerializer();
                }
                return instance;
            }
        }
        #endregion

        #region Static Fields
        private static ULongSerializer instance;
        #endregion

        #region Constructors
        private ULongSerializer() : base(typeof(ulong))
        {
        }
        #endregion

        #region Methods
        public void Serialize(string primitiveAttributeName, ulong primitiveAttribute, out XAttribute serializedPrimitiveAttribute)
        {
            serializedPrimitiveAttribute = new XAttribute(primitiveAttributeName, primitiveAttribute);
        }

        public void Deserialize(XAttribute serializedPrimitiveAttribute, out ulong primitiveAttribute)
        {
            if (serializedPrimitiveAttribute == null)
            {
                throw new ArgumentNullException(nameof(serializedPrimitiveAttribute));
            }

            if (!ulong.TryParse(serializedPrimitiveAttribute.Value, out primitiveAttribute))
            {
                throw new ArgumentException($"Attribute '{serializedPrimitiveAttribute.Name}' with value '{serializedPrimitiveAttribute.Value}' could not be parsed as a ulong!");
            }
        }
        #endregion

        #region Overrides
        public override void Serialize(string primitiveAttributeName, object primitiveAttribute, out XAttribute serializedPrimitiveAttribute) => Serialize(primitiveAttributeName, (ulong)primitiveAttribute, out serializedPrimitiveAttribute);
        public override void Deserialize(XAttribute serializedPrimitiveAttribute, out object primitiveAttribute) => Deserialize(serializedPrimitiveAttribute, out primitiveAttribute);
        #endregion
    }
}
