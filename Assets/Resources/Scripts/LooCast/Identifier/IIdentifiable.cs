using LooCast.Core;
using System;
using System.Reflection;
using System.Collections.Generic;
using UnityEngine;

namespace LooCast.Identifier
{
    public interface IIdentifiable
    {
        #region Properties
        IIdentifier Identifier { get; }
        #endregion
    }
}