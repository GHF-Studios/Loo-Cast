using System;
using System.Collections.Generic;
using System.Reflection;
using System.Xml.Linq;

namespace LooCast.System.Serialization
{
    public sealed class NonGenericObjectTypeInfo : ObjectTypeInfo
    {
        #region Properties
        public PropertyInfo[] Properties { get; set; }
        public FieldInfo[] Fields { get; set; }
        
        public bool OverrideSerialization { get; set; }
        public bool OverrideDeserialization { get; set; }
        #endregion

        #region Constructors
        public NonGenericObjectTypeInfo(Type type) : base(type, Serializability.NonGenericObject)
        {
        }
        #endregion
    }
}
