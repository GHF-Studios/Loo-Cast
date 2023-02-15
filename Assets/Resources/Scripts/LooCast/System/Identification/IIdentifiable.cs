namespace LooCast.System.Identification
{
    public interface IIdentifiable
    {
        #region Properties
        IIdentifier Identifier { get; }
        #endregion
    }
}