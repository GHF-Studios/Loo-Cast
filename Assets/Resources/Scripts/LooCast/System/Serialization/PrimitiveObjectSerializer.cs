using System;
using System.Xml.Linq;

namespace LooCast.System.Serialization
{
    public abstract class PrimitiveObjectSerializer : IPrimitiveObjectSerializer
    {
        #region Properties
        public Type PrimitiveObjectType { get; private set; }
        #endregion

        #region Constructors
        protected PrimitiveObjectSerializer(Type primitiveObjectType)
        {
            if (primitiveObjectType == null)
            {
                throw new ArgumentNullException(nameof(primitiveObjectType));
            }

            PrimitiveObjectType = primitiveObjectType;
        }
        #endregion

        #region Methods
        public abstract void Serialize(string primitiveObjectName, object primitiveObject, out XElement serializedPrimitiveObject);
        public abstract void Deserialize(XElement serializedPrimitiveObject, out object primitiveObject);
        #endregion
    }
}
