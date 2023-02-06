using System;
using System.Collections.Generic;

namespace LooCast.Data_NEW
{
    public interface IDataLayer
    {
        string Name { get; set; }
        IDataLayer Parent { get; set; }
        IList<IDataLayer> Children { get; set; }
    }
}
