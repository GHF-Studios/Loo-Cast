﻿using System.Collections.Generic;

namespace LooCast.System
{
    using LooCast.System.MetaData;
    using LooCast.System.Data;

    public interface IType : ILooCastObject
    {
        #region Properties
        public ITypeMetaData TypeMetaData { get; set; }
        public ITypeData TypeData { get; set; }
        #endregion
    }
}
