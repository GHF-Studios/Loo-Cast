using System;
using System.Numerics;
using System.Xml.Linq;

namespace LooCast.System.Serialization
{
    public sealed class CharSerializer : PrimitiveAttributeSerializer, IPrimitiveAttributeSerializer<char>
    {
        #region Static Properties
        public static CharSerializer Instance
        {
            get
            {
                if (instance == null)
                {
                    instance = new CharSerializer();
                }
                return instance;
            }
        }
        #endregion

        #region Static Fields
        private static CharSerializer instance;
        #endregion

        #region Constructors
        private CharSerializer() : base(typeof(char))
        {
        }
        #endregion

        #region Methods
        public void Serialize(string primitiveAttributeName, char primitiveAttribute, out XAttribute serializedPrimitiveAttribute)
        {
            serializedPrimitiveAttribute = new XAttribute(primitiveAttributeName, primitiveAttribute);
        }

        public void Deserialize(XAttribute serializedPrimitiveAttribute, out char primitiveAttribute)
        {
            if (serializedPrimitiveAttribute == null)
            {
                throw new ArgumentNullException(nameof(serializedPrimitiveAttribute));
            }

            if (!char.TryParse(serializedPrimitiveAttribute.Value, out primitiveAttribute))
            {
                throw new ArgumentException($"Attribute '{serializedPrimitiveAttribute.Name}' with value '{serializedPrimitiveAttribute.Value}' could not be parsed as a char!");
            }
        }
        #endregion

        #region Overrides
        public override void Serialize(string primitiveAttributeName, object primitiveAttribute, out XAttribute serializedPrimitiveAttribute) => Serialize(primitiveAttributeName, (char)primitiveAttribute, out serializedPrimitiveAttribute);
        public override void Deserialize(XAttribute serializedPrimitiveAttribute, out object primitiveAttribute) => Deserialize(serializedPrimitiveAttribute, out primitiveAttribute);
        #endregion
    }
}
