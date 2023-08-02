using System;
using System.Numerics;
using System.Xml.Linq;

namespace LooCast.System.Serialization
{
    public sealed class BoolSerializer : PrimitiveAttributeSerializer, IPrimitiveAttributeSerializer<bool>
    {
        #region Static Properties
        public static BoolSerializer Instance
        {
            get
            {
                if (instance == null)
                {
                    instance = new BoolSerializer();
                }
                return instance;
            }
        }
        #endregion

        #region Static Fields
        private static BoolSerializer instance;
        #endregion

        #region Constructors
        private BoolSerializer() : base(typeof(bool))
        {
        }
        #endregion

        #region Methods
        public void Serialize(string primitiveAttributeName, bool primitiveAttribute, out XAttribute serializedPrimitiveAttribute)
        {
            serializedPrimitiveAttribute = new XAttribute(primitiveAttributeName, primitiveAttribute);
        }

        public void Deserialize(XAttribute serializedPrimitiveAttribute, out bool primitiveAttribute)
        {
            if (serializedPrimitiveAttribute == null)
            {
                throw new ArgumentNullException(nameof(serializedPrimitiveAttribute));
            }

            if (!bool.TryParse(serializedPrimitiveAttribute.Value, out primitiveAttribute))
            {
                throw new ArgumentException($"Attribute '{serializedPrimitiveAttribute.Name}' with value '{serializedPrimitiveAttribute.Value}' could not be parsed as a bool!");
            }
        }
        #endregion
        
        #region Overrides
        public override void Serialize(string primitiveAttributeName, object primitiveAttribute, out XAttribute serializedPrimitiveAttribute) => Serialize(primitiveAttributeName, (bool)primitiveAttribute, out serializedPrimitiveAttribute);
        public override void Deserialize(XAttribute serializedPrimitiveAttribute, out object primitiveAttribute) => Deserialize(serializedPrimitiveAttribute, out primitiveAttribute);
        #endregion
    }
}
