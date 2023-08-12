using System;
using System.Collections.Generic;
using System.Xml.Linq;

namespace LooCast.System.Serialization
{
    public abstract class ObjectTypeInfo : TypeInfo
    {
        #region Delegates
        public delegate void Serialize(string objectName, object _object, out XElement serializedObject);
        public delegate void Deserialize(XElement serializedObject, out object _object);
        #endregion
        
        #region Properties
        public HashSet<PrimitiveTypeInfo> PrimitiveTypeDependencies { get; set; }
        public HashSet<NonGenericObjectTypeInfo> NonGenericObjectTypeDependencies { get; set; }
        public HashSet<GenericObjectTypeInfo> GenericObjectTypeDependencies { get; set; }

        public Serialize SerializeDelegate { get; set; }
        public Deserialize DeserializeDelegate { get; set; }
        #endregion

        #region Constructors
        protected ObjectTypeInfo(Type type, Serializability serializability) : base(type, serializability)
        {
        }
        #endregion
    }
}
