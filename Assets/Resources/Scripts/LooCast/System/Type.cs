using CSSystem = System;
using System.Collections.Generic;

namespace LooCast.System
{
    using LooCast.System.Data;
    using LooCast.System.Identifiers;
    using LooCast.System.MetaData;
    using LooCast.System.Registries;

    public abstract class Type<TInstance> : IType
        where TInstance : ILooCastObject, new()
    {
        #region Properties
        public abstract IMetaData MetaData { get; set; }
        public abstract ITypeMetaData TypeMetaData { get; set; }
        
        public abstract IData Data { get; set; }
        public abstract ITypeData TypeData { get; set; }
        #endregion

        #region Methods
        public abstract bool Validate();
        #endregion
    }
}
