// Fill out your copyright notice in the Description page of Project Settings.

#include "CaptainFunctionalPawn.h"


// Sets default values
ACaptainFunctionalPawn::ACaptainFunctionalPawn()
{
	// Set this pawn to call Tick() every frame.  You can turn this off to improve performance if you don't need it.
	PrimaryActorTick.bCanEverTick = true;

}

// Called when the game starts or when spawned
void ACaptainFunctionalPawn::BeginPlay()
{
	Super::BeginPlay();


}

// Called every frame
void ACaptainFunctionalPawn::Tick(float DeltaTime)
{
	Super::Tick(DeltaTime);

	
}

// Called to bind functionality to input
void ACaptainFunctionalPawn::SetupPlayerInputComponent(UInputComponent* PlayerInputComponent)
{
	Super::SetupPlayerInputComponent(PlayerInputComponent);

	
}

int ACaptainFunctionalPawn::GetHealth()
{
	return health;
}

void ACaptainFunctionalPawn::SetHealth(int hp)
{
	health = hp;
	if (health <= 0) {
		// TODO: Death
	}
}
