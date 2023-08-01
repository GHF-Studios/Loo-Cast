using System;
using System.Xml.Linq;

namespace LooCast.System.Serialization
{
    public interface IPrimitiveObjectSerializer
    {
        #region Methods
        public void Serialize(object primitiveObject, out XElement serializedPrimitiveObject);
        public void Deserialize(XElement serializedPrimitiveObject, out object primitiveObject);
        #endregion
    }
    
    public interface IPrimitiveObjectSerializer<PrimitiveObjectType> : IPrimitiveObjectSerializer
    {
        #region Methods
        public void Serialize(PrimitiveObjectType primitiveObject, out XElement serializedPrimitiveObject);
        public void Deserialize(XElement serializedPrimitiveObject, out PrimitiveObjectType primitiveObject);
        #endregion
    }
}
