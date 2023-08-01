using System;
using System.Xml.Linq;

namespace LooCast.System.Serialization
{
    public interface IPrimitiveObjectSerializer
    {
        #region Methods
        public void Serialize(string primitiveObjectName, object primitiveObject, out XElement serializedPrimitiveObject);
        public void Deserialize(XElement serializedPrimitiveObject, out object primitiveObject);
        #endregion
    }
    
    public interface IPrimitiveObjectSerializer<PrimitiveObjectType> : IPrimitiveObjectSerializer
    {
        #region Methods
        public void Serialize(string primitiveObjectName, PrimitiveObjectType primitiveObject, out XElement serializedPrimitiveObject);
        public void Deserialize(XElement serializedPrimitiveObject, out PrimitiveObjectType primitiveObject);
        #endregion
    }
}
