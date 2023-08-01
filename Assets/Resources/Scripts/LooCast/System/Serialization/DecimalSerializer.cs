using System;
using System.Xml.Linq;

namespace LooCast.System.Serialization
{
    public sealed class DecimalSerializer : IPrimitiveAttributeSerializer<decimal>
    {
        #region Static Properties
        public static DecimalSerializer Instance
        {
            get
            {
                if (instance == null)
                {
                    instance = new DecimalSerializer();
                }
                return instance;
            }
        }
        #endregion

        #region Static Fields
        private static DecimalSerializer instance;
        #endregion

        #region Constructors
        private DecimalSerializer() : base()
        {
        }
        #endregion

        #region Methods
        public void Serialize(string primitiveAttributeName, object primitiveAttribute, out XAttribute serializedPrimitiveAttribute) => Serialize(primitiveAttributeName, (decimal)primitiveAttribute, out serializedPrimitiveAttribute);

        public void Serialize(string primitiveAttributeName, decimal primitiveAttribute, out XAttribute serializedPrimitiveAttribute)
        {
            serializedPrimitiveAttribute = new XAttribute(primitiveAttributeName, primitiveAttribute);
        }

        public void Deserialize(XAttribute serializedPrimitiveAttribute, out object primitiveAttribute) => Deserialize(serializedPrimitiveAttribute, out primitiveAttribute);

        public void Deserialize(XAttribute serializedPrimitiveAttribute, out decimal primitiveAttribute)
        {
            if (serializedPrimitiveAttribute == null)
            {
                throw new ArgumentNullException(nameof(serializedPrimitiveAttribute));
            }

            if (!decimal.TryParse(serializedPrimitiveAttribute.Value, out primitiveAttribute))
            {
                throw new ArgumentException($"Attribute '{serializedPrimitiveAttribute.Name}' with value '{serializedPrimitiveAttribute.Value}' could not be parsed as a decimal!");
            }
        }
        #endregion
    }
}
