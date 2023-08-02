using System;
using System.Xml.Linq;

namespace LooCast.System.Serialization
{
    public sealed class UShortSerializer : PrimitiveAttributeSerializer, IPrimitiveAttributeSerializer<ushort>
    {
        #region Static Properties
        public static UShortSerializer Instance
        {
            get
            {
                if (instance == null)
                {
                    instance = new UShortSerializer();
                }
                return instance;
            }
        }
        #endregion

        #region Static Fields
        private static UShortSerializer instance;
        #endregion

        #region Constructors
        private UShortSerializer() : base(typeof(ushort))
        {
        }
        #endregion

        #region Methods
        public void Serialize(string primitiveAttributeName, ushort primitiveAttribute, out XAttribute serializedPrimitiveAttribute)
        {
            serializedPrimitiveAttribute = new XAttribute(primitiveAttributeName, primitiveAttribute);
        }

        public void Deserialize(XAttribute serializedPrimitiveAttribute, out ushort primitiveAttribute)
        {
            if (serializedPrimitiveAttribute == null)
            {
                throw new ArgumentNullException(nameof(serializedPrimitiveAttribute));
            }

            if (!ushort.TryParse(serializedPrimitiveAttribute.Value, out primitiveAttribute))
            {
                throw new ArgumentException($"Attribute '{serializedPrimitiveAttribute.Name}' with value '{serializedPrimitiveAttribute.Value}' could not be parsed as a ushort!");
            }
        }
        #endregion

        #region Overrides
        public override void Serialize(string primitiveAttributeName, object primitiveAttribute, out XAttribute serializedPrimitiveAttribute) => Serialize(primitiveAttributeName, (ushort)primitiveAttribute, out serializedPrimitiveAttribute);
        public override void Deserialize(XAttribute serializedPrimitiveAttribute, out object primitiveAttribute) => Deserialize(serializedPrimitiveAttribute, out primitiveAttribute);
        #endregion
    }
}
