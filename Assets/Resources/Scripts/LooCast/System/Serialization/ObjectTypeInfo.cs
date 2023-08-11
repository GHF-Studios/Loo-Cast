using System;
using System.Collections.Generic;
using System.Xml.Linq;

namespace LooCast.System.Serialization
{
    public abstract class ObjectTypeInfo
    {
        #region Delegates
        public delegate void Serialize(string objectName, object _object, out XElement serializedObject);
        public delegate void Deserialize(XElement serializedObject, out object _object);
        #endregion

        #region Classes
        public abstract class PreAnalysisInfo
        {
            #region Properties
            public bool IsGeneric { get; private set; }
            #endregion

            #region Constructors
            protected PreAnalysisInfo(bool isGeneric)
            {
                IsGeneric = isGeneric;
            }
            #endregion
        }

        public abstract class AnalysisInfo
        {
            #region Properties
            public HashSet<PrimitiveTypeInfo> PrimitiveTypeDependencies { get; private set; }
            public HashSet<ObjectTypeInfo> ObjectTypeDependencies { get; private set; }
            #endregion

            #region Constructors
            protected AnalysisInfo(HashSet<PrimitiveTypeInfo> primitiveTypeDependencies, HashSet<ObjectTypeInfo> objectTypeDependencies)
            {
                PrimitiveTypeDependencies = primitiveTypeDependencies;
                ObjectTypeDependencies = objectTypeDependencies;
            }
            #endregion
        }

        public abstract class ProcessingInfo
        {
            #region Properties
            public Serialize SerializeDelegate { get; private set; }
            public Deserialize DeserializeDelegate { get; private set; }
            #endregion

            #region Constructors
            protected ProcessingInfo(Serialize serializeDelegate, Deserialize deserializeDelegate)
            {
                SerializeDelegate = serializeDelegate;
                DeserializeDelegate = deserializeDelegate;
            }
            #endregion
        }
        #endregion
    }
}
