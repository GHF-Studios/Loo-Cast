using System;
using System.Xml.Linq;

namespace LooCast.System.Serialization
{
    public sealed class FloatSerializer : IPrimitiveAttributeSerializer<float>
    {
        #region Static Properties
        public static FloatSerializer Instance
        {
            get
            {
                if (instance == null)
                {
                    instance = new FloatSerializer();
                }
                return instance;
            }
        }
        #endregion

        #region Static Fields
        private static FloatSerializer instance;
        #endregion

        #region Constructors
        private FloatSerializer() : base()
        {
        }
        #endregion

        #region Methods
        public void Serialize(object primitiveAttribute, out XAttribute serializedPrimitiveAttribute) => Serialize((float)primitiveAttribute, out serializedPrimitiveAttribute);

        public void Serialize(float primitiveAttribute, out XAttribute serializedPrimitiveAttribute)
        {
            serializedPrimitiveAttribute = new XAttribute(nameof(primitiveAttribute), primitiveAttribute);
        }

        public void Deserialize(XAttribute serializedPrimitiveAttribute, out object primitiveAttribute) => Deserialize(serializedPrimitiveAttribute, out primitiveAttribute);

        public void Deserialize(XAttribute serializedPrimitiveAttribute, out float primitiveAttribute)
        {
            if (serializedPrimitiveAttribute == null)
            {
                throw new ArgumentNullException(nameof(serializedPrimitiveAttribute));
            }

            if (!float.TryParse(serializedPrimitiveAttribute.Value, out primitiveAttribute))
            {
                throw new ArgumentException($"Attribute '{serializedPrimitiveAttribute.Name}' with value '{serializedPrimitiveAttribute.Value}' could not be parsed as a float!");
            }
        }
        #endregion
    }
}
