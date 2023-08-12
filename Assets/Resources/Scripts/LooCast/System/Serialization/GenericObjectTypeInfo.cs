using System;
using System.Collections.Generic;
using System.Xml.Linq;

namespace LooCast.System.Serialization
{
    public sealed class GenericObjectTypeInfo : ObjectTypeInfo
    {
        #region Constructors
        public GenericObjectTypeInfo(Type type) : base(type, Serializability.GenericObject)
        {
        }
        #endregion
    }
}
