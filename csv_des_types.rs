#![allow(unused)]

enum MSZoning{CSpaceOpenParenallCloseParen,}

enum Street{Pave,Grvl,}

enum Alley{Pave,Grvl,}

enum LotShape{IR3,Reg,IR1,IR2,}

enum LandContour{Low,HLS,Lvl,Bnk,}

enum Utilities{NoSeWa,AllPub,}

enum LotConfig{Inside,FR2,Corner,FR3,CulDSac,}

enum LandSlope{Mod,Gtl,Sev,}

enum Neighborhood{Blmngtn,SWISU,NAmes,Sawyer,MeadowV,NridgHt,Blueste,Gilbert,Crawfor,NWAmes,SawyerW,Mitchel,NPkVill,Somerst,BrkSide,Veenker,BrDale,CollgCr,NoRidge,ClearCr,StoneBr,OldTown,Timber,IDOTRR,Edwards,}

enum Condition1{Artery,RRNe,RRAe,RRAn,Norm,Feedr,PosA,PosN,RRNn,}

enum Condition2{Feedr,RRNn,Norm,PosA,RRAe,Artery,PosN,RRAn,}

enum BldgType{TwofmCon,OneFam,}

enum HouseStyle{OneStory,OnePointFiveUnf,TwoPointFiveFin,TwoPointFiveUnf,TwoStory,OnePointFiveFin,}

enum RoofStyle{Hip,Gable,Flat,Gambrel,Shed,Mansard,}

enum RoofMatl{TarAndGrv,}

enum Exterior1st{WdSpaceSdng,}

enum Exterior2nd{BrkSpaceCmn,WdSpaceSdng,WdSpaceShng,}

enum MasVnrType{None,Stone,BrkFace,BrkCmn,}

enum ExterQual{Gd,Ex,Fa,TA,}

enum ExterCond{Gd,Fa,Po,TA,Ex,}

enum Foundation{BrkTil,Slab,Wood,CBlock,Stone,PConc,}

enum BsmtQual{Gd,TA,Ex,Fa,}

enum BsmtCond{Gd,TA,Po,Fa,}

enum BsmtExposure{Av,Gd,Mn,No,}

enum BsmtFinType1{ALQ,BLQ,Unf,LwQ,GLQ,Rec,}

enum BsmtFinType2{LwQ,BLQ,Rec,Unf,GLQ,ALQ,}

enum Heating{Wall,Floor,Grav,GasW,OthW,GasA,}

enum HeatingQC{Ex,Po,Gd,Fa,TA,}

enum CentralAir{Y,N,}

enum Electrical{FuseA,FuseF,SBrkr,FuseP,Mix,}

enum KitchenQual{Fa,TA,Gd,Ex,}

enum Functional{Maj1,Sev,Maj2,Mod,Typ,Min2,Min1,}

enum FireplaceQu{TA,Po,Gd,Fa,Ex,}

enum GarageType{TwoTypes,}

enum GarageFinish{RFn,Fin,Unf,}

enum GarageQual{Gd,Po,Fa,TA,Ex,}

enum GarageCond{Fa,Po,Ex,TA,Gd,}

enum PavedDrive{P,Y,N,}

enum PoolQC{Gd,Fa,Ex,}

enum Fence{GdWo,MnWw,MnPrv,GdPrv,}

enum MiscFeature{Shed,TenC,Gar2,Othr,}

enum SaleType{Oth,COD,WD,ConLw,Con,ConLI,ConLD,New,CWD,}

enum SaleCondition{Family,AdjLand,Abnorml,Normal,Alloca,Partial,}

struct SerdeCsvDeserializer{mszoning : MSZoning,street : Street,alley : Alley,lotshape : LotShape,landcontour : LandContour,utilities : Utilities,lotconfig : LotConfig,landslope : LandSlope,neighborhood : Neighborhood,condition1 : Condition1,condition2 : Condition2,bldgtype : BldgType,housestyle : HouseStyle,roofstyle : RoofStyle,roofmatl : RoofMatl,exterior1st : Exterior1st,exterior2nd : Exterior2nd,masvnrtype : MasVnrType,exterqual : ExterQual,extercond : ExterCond,foundation : Foundation,bsmtqual : BsmtQual,bsmtcond : BsmtCond,bsmtexposure : BsmtExposure,bsmtfintype1 : BsmtFinType1,bsmtfintype2 : BsmtFinType2,heating : Heating,heatingqc : HeatingQC,centralair : CentralAir,electrical : Electrical,kitchenqual : KitchenQual,functional : Functional,fireplacequ : FireplaceQu,garagetype : GarageType,garagefinish : GarageFinish,garagequal : GarageQual,garagecond : GarageCond,paveddrive : PavedDrive,poolqc : PoolQC,fence : Fence,miscfeature : MiscFeature,saletype : SaleType,salecondition : SaleCondition,}