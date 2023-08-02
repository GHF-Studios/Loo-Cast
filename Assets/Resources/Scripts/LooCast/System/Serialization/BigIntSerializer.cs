using System;
using System.Numerics;
using System.Xml.Linq;

namespace LooCast.System.Serialization
{
    public sealed class BigIntSerializer : PrimitiveAttributeSerializer, IPrimitiveAttributeSerializer<BigInteger>
    {
        #region Static Properties
        public static BigIntSerializer Instance
        {
            get
            {
                if (instance == null)
                {
                    instance = new BigIntSerializer();
                }
                return instance;
            }
        }
        #endregion

        #region Static Fields
        private static BigIntSerializer instance;
        #endregion

        #region Constructors
        private BigIntSerializer() : base(typeof(BigInteger))
        {
        }
        #endregion

        #region Methods
        public void Serialize(string primitiveAttributeName, BigInteger primitiveAttribute, out XAttribute serializedPrimitiveAttribute)
        {
            serializedPrimitiveAttribute = new XAttribute(primitiveAttributeName, primitiveAttribute);
        }

        public void Deserialize(XAttribute serializedPrimitiveAttribute, out BigInteger primitiveAttribute)
        {
            if (serializedPrimitiveAttribute == null)
            {
                throw new ArgumentNullException(nameof(serializedPrimitiveAttribute));
            }

            if (!BigInteger.TryParse(serializedPrimitiveAttribute.Value, out primitiveAttribute))
            {
                throw new ArgumentException($"Attribute '{serializedPrimitiveAttribute.Name}' with value '{serializedPrimitiveAttribute.Value}' could not be parsed as a BigInteger!");
            }
        }
        #endregion

        #region Overrides
        public override void Serialize(string primitiveAttributeName, object primitiveAttribute, out XAttribute serializedPrimitiveAttribute) => Serialize(primitiveAttributeName, (BigInteger)primitiveAttribute, out serializedPrimitiveAttribute);
        public override void Deserialize(XAttribute serializedPrimitiveAttribute, out object primitiveAttribute) => Deserialize(serializedPrimitiveAttribute, out primitiveAttribute);
        #endregion
    }
}
