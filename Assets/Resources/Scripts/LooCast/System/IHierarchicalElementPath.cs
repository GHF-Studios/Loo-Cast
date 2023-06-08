using System;
using UnityEngine;

namespace LooCast.System
{
    public interface IHierarchicalElementPath
    {
        #region Properties
        string GUSP { get; }
        bool IsRelative { get; }
        #endregion
    }
}
