using System;
using System.Collections.Generic;

namespace LooCast.System
{
    public interface IReferenceData
    {
        #region Properties
        public HashSet<IReferenceData> ChildReferenceData { get; }
        public HashSet<IValueData> ChildValueData { get; }
        #endregion
    }
}
