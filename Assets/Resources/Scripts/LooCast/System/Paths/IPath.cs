using System;
using UnityEngine;

namespace LooCast.System.Paths
{
    public interface IPath
    {
        #region Properties
        string GUSP { get; }
        bool IsRelative { get; }
        PathType PathType { get; }
        #endregion
    }
}
