using System;
using System.Xml.Linq;

namespace LooCast.System.Serialization
{
    public sealed class CharSerializer : IPrimitiveAttributeSerializer<char>
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
        private CharSerializer() : base()
        {
        }
        #endregion

        #region Methods
        public void Serialize(object primitiveAttribute, out XAttribute serializedPrimitiveAttribute) => Serialize((char)primitiveAttribute, out serializedPrimitiveAttribute);

        public void Serialize(char primitiveAttribute, out XAttribute serializedPrimitiveAttribute)
        {
            serializedPrimitiveAttribute = new XAttribute(nameof(primitiveAttribute), primitiveAttribute);
        }

        public void Deserialize(XAttribute serializedPrimitiveAttribute, out object primitiveAttribute) => Deserialize(serializedPrimitiveAttribute, out primitiveAttribute);

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
    }
}
