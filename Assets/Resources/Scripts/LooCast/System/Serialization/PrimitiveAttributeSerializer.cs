using System;
using System.Xml.Linq;

namespace LooCast.System.Serialization
{
    public abstract class PrimitiveAttributeSerializer : IPrimitiveAttributeSerializer
    {
        #region Properties
        public Type PrimitiveAttributeType { get; private set; }
        #endregion

        #region Constructors
        protected PrimitiveAttributeSerializer(Type primitiveAttributeType)
        {
            if (primitiveAttributeType == null)
            {
                throw new ArgumentNullException(nameof(primitiveAttributeType));
            }

            PrimitiveAttributeType = primitiveAttributeType;
        }
        #endregion

        #region Methods
        public abstract void Serialize(string primitiveAttributeName, object primitiveAttribute, out XAttribute serializedPrimitiveAttribute);
        public abstract void Deserialize(XAttribute serializedPrimitiveAttribute, out object primitiveAttribute);
        #endregion
    }
}
