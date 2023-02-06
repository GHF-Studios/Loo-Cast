using System;
using UnityEngine;

namespace LooCast.API
{
    public interface IAPI
    {
        #region Properties
        string ModName { get; }
        string ModuleName { get; }
        string Name { get; }
        #endregion

        #region Methods
        virtual void OnPreInitialize()
        {
            
        }

        virtual void OnInitialize()
        {
            
        }

        virtual void OnPostInitialize()
        {
            
        }
        #endregion
    }
}
