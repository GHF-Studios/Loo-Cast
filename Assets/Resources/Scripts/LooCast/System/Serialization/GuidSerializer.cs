using System;
using System.Numerics;
using System.Xml.Linq;

namespace LooCast.System.Serialization
{
    public sealed class GuidSerializer : PrimitiveAttributeSerializer, IPrimitiveAttributeSerializer<Guid>
    {
        #region Static Properties
        public static GuidSerializer Instance
        {
            get
            {
                if (instance == null)
                {
                    instance = new GuidSerializer();
                }
                return instance;
            }
        }
        #endregion

        #region Static Fields
        private static GuidSerializer instance;
        #endregion

        #region Constructors
        private GuidSerializer() : base(typeof(Guid))
        {
        }
        #endregion

        #region Methods
        public void Serialize(string primitiveAttributeName, Guid primitiveAttribute, out XAttribute serializedPrimitiveAttribute)
        {
            serializedPrimitiveAttribute = new XAttribute(primitiveAttributeName, primitiveAttribute);
        }

        public void Deserialize(XAttribute serializedPrimitiveAttribute, out Guid primitiveAttribute)
        {
            if (serializedPrimitiveAttribute == null)
            {
                throw new ArgumentNullException(nameof(serializedPrimitiveAttribute));
            }

            if (!Guid.TryParse(serializedPrimitiveAttribute.Value, out primitiveAttribute))
            {
                throw new ArgumentException($"Attribute '{serializedPrimitiveAttribute.Name}' with value '{serializedPrimitiveAttribute.Value}' could not be parsed as a Guid!");
            }
        }
        #endregion

        #region Overrides
        public override void Serialize(string primitiveAttributeName, object primitiveAttribute, out XAttribute serializedPrimitiveAttribute) => Serialize(primitiveAttributeName, (Guid)primitiveAttribute, out serializedPrimitiveAttribute);
        public override void Deserialize(XAttribute serializedPrimitiveAttribute, out object primitiveAttribute) => Deserialize(serializedPrimitiveAttribute, out primitiveAttribute);
        #endregion
    }
}
