using System;
using System.Xml.Linq;

namespace LooCast.System.Serialization
{
    public sealed class UIntSerializer : PrimitiveAttributeSerializer, IPrimitiveAttributeSerializer<uint>
    {
        #region Static Properties
        public static UIntSerializer Instance
        {
            get
            {
                if (instance == null)
                {
                    instance = new UIntSerializer();
                }
                return instance;
            }
        }
        #endregion

        #region Static Fields
        private static UIntSerializer instance;
        #endregion

        #region Constructors
        private UIntSerializer() : base(typeof(uint))
        {
        }
        #endregion

        #region Methods
        public void Serialize(string primitiveAttributeName, uint primitiveAttribute, out XAttribute serializedPrimitiveAttribute)
        {
            serializedPrimitiveAttribute = new XAttribute(primitiveAttributeName, primitiveAttribute);
        }

        public void Deserialize(XAttribute serializedPrimitiveAttribute, out uint primitiveAttribute)
        {
            if (serializedPrimitiveAttribute == null)
            {
                throw new ArgumentNullException(nameof(serializedPrimitiveAttribute));
            }

            if (!uint.TryParse(serializedPrimitiveAttribute.Value, out primitiveAttribute))
            {
                throw new ArgumentException($"Attribute '{serializedPrimitiveAttribute.Name}' with value '{serializedPrimitiveAttribute.Value}' could not be parsed as a uint!");
            }
        }
        #endregion

        #region Overrides
        public override void Serialize(string primitiveAttributeName, object primitiveAttribute, out XAttribute serializedPrimitiveAttribute) => Serialize(primitiveAttributeName, (uint)primitiveAttribute, out serializedPrimitiveAttribute);
        public override void Deserialize(XAttribute serializedPrimitiveAttribute, out object primitiveAttribute) => Deserialize(serializedPrimitiveAttribute, out primitiveAttribute);
        #endregion
    }
}
