using System;
using System.Numerics;
using System.Xml.Linq;

namespace LooCast.System.Serialization
{
    public sealed class DoubleSerializer : PrimitiveAttributeSerializer, IPrimitiveAttributeSerializer<double>
    {
        #region Static Properties
        public static DoubleSerializer Instance
        {
            get
            {
                if (instance == null)
                {
                    instance = new DoubleSerializer();
                }
                return instance;
            }
        }
        #endregion

        #region Static Fields
        private static DoubleSerializer instance;
        #endregion

        #region Constructors
        private DoubleSerializer() : base(typeof(double))
        {
        }
        #endregion

        #region Methods
        public void Serialize(string primitiveAttributeName, double primitiveAttribute, out XAttribute serializedPrimitiveAttribute)
        {
            serializedPrimitiveAttribute = new XAttribute(primitiveAttributeName, primitiveAttribute);
        }

        public void Deserialize(XAttribute serializedPrimitiveAttribute, out double primitiveAttribute)
        {
            if (serializedPrimitiveAttribute == null)
            {
                throw new ArgumentNullException(nameof(serializedPrimitiveAttribute));
            }

            if (!double.TryParse(serializedPrimitiveAttribute.Value, out primitiveAttribute))
            {
                throw new ArgumentException($"Attribute '{serializedPrimitiveAttribute.Name}' with value '{serializedPrimitiveAttribute.Value}' could not be parsed as a double!");
            }
        }
        #endregion

        #region Overrides
        public override void Serialize(string primitiveAttributeName, object primitiveAttribute, out XAttribute serializedPrimitiveAttribute) => Serialize(primitiveAttributeName, (double)primitiveAttribute, out serializedPrimitiveAttribute);
        public override void Deserialize(XAttribute serializedPrimitiveAttribute, out object primitiveAttribute) => Deserialize(serializedPrimitiveAttribute, out primitiveAttribute);
        #endregion
    }
}
