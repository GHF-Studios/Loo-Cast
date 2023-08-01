using System;
using System.Xml.Linq;

namespace LooCast.System.Serialization
{
    public sealed class IntSerializer : IPrimitiveAttributeSerializer<int>
    {
        #region Static Properties
        public static IntSerializer Instance
        {
            get
            {
                if (instance == null)
                {
                    instance = new IntSerializer();
                }
                return instance;
            }
        }
        #endregion

        #region Static Fields
        private static IntSerializer instance;
        #endregion

        #region Constructors
        private IntSerializer() : base()
        {
        }
        #endregion

        #region Methods
        public void Serialize(object primitiveAttribute, out XAttribute serializedPrimitiveAttribute) => Serialize((int)primitiveAttribute, out serializedPrimitiveAttribute);

        public void Serialize(int primitiveAttribute, out XAttribute serializedPrimitiveAttribute)
        {
            serializedPrimitiveAttribute = new XAttribute(nameof(primitiveAttribute), primitiveAttribute);
        }

        public void Deserialize(XAttribute serializedPrimitiveAttribute, out object primitiveAttribute) => Deserialize(serializedPrimitiveAttribute, out primitiveAttribute);

        public void Deserialize(XAttribute serializedPrimitiveAttribute, out int primitiveAttribute)
        {
            if (serializedPrimitiveAttribute == null)
            {
                throw new ArgumentNullException(nameof(serializedPrimitiveAttribute));
            }

            if (!int.TryParse(serializedPrimitiveAttribute.Value, out primitiveAttribute))
            {
                throw new ArgumentException($"Attribute '{serializedPrimitiveAttribute.Name}' with value '{serializedPrimitiveAttribute.Value}' could not be parsed as an int!");
            }
        }
        #endregion
    }
}
