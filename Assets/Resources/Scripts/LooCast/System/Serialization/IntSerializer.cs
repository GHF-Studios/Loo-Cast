using System;
using System.Xml.Linq;

namespace LooCast.System.Serialization
{
    public sealed class IntSerializer : PrimitiveAttributeSerializer, IPrimitiveAttributeSerializer<int>
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
        private IntSerializer() : base(typeof(int))
        {
        }
        #endregion

        #region Methods
        public void Serialize(string primitiveAttributeName, int primitiveAttribute, out XAttribute serializedPrimitiveAttribute)
        {
            serializedPrimitiveAttribute = new XAttribute(primitiveAttributeName, primitiveAttribute);
        }

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

        #region Overrides
        public override void Serialize(string primitiveAttributeName, object primitiveAttribute, out XAttribute serializedPrimitiveAttribute) => Serialize(primitiveAttributeName, (int)primitiveAttribute, out serializedPrimitiveAttribute);
        public override void Deserialize(XAttribute serializedPrimitiveAttribute, out object primitiveAttribute) => Deserialize(serializedPrimitiveAttribute, out primitiveAttribute);
        #endregion
    }
}
